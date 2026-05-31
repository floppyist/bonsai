#!/usr/bin/env python3

import json
import base64
import io
import os
import sys
import glob
from typing import List
import numpy as np
import cv2
import ollama
from PIL import Image, ImageOps

# ---------------------------
# CONFIG
# ---------------------------
OLLAMA_MODEL = "qwen2.5vl:3b"  
DEBUG_DIR = "debug_images"

# ---------------------------
# MAXIMALES PREPROCESSING
# ---------------------------
def maximize_text_quality(img_np: np.ndarray) -> np.ndarray:
    """
    Entfernt Schattenwürfe, eliminiert das Durchscheinen der Rückseite (Ghosting)
    und maximiert den Textkontrast für das VLM.
    """
    print("[DEBUG] Starte Text-Optimierung...")
    
    # 1. In Graustufen wandeln
    gray = cv2.cvtColor(img_np, cv2.COLOR_RGB2GRAY)
    
    # 2. Schatten- & Ausleuchtungs-Korrektur (Illumination Correction)
    # Erzeugt eine Schätzung des Hintergrunds durch morphologisches Öffnen
    kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (50, 50))
    background = cv2.morphologyEx(gray, cv2.MORPH_DILATE, kernel)
    background = cv2.GaussianBlur(background, (51, 51), 0)
    
    # Division des Originalbildes durch den Hintergrund entfernt Helligkeitsverläufe
    normalized = cv2.divide(gray, background, scale=255)
    
    # 3. Bilateral Filter gegen Körnigkeit (Schriftkanten bleiben scharf)
    denoised = cv2.bilateralFilter(normalized, d=7, sigmaColor=50, sigmaSpace=50)
    
    # 4. GHOSTING-FILTER (Schwellenwert für helle Grauwerte)
    # Alles, was heller als 200 ist (Skala 0-255), wird zu reinem Weiß (255) gesetzt.
    # Das schneidet das schwache Durchscheinen der Rückseite effektiv ab.
    # Falls noch Reste da sind, den Wert schrittweise senken (z.B. auf 190 oder 185).
    WHITE_THRESHOLD = 200 
    _, ghosting_mask = cv2.threshold(denoised, WHITE_THRESHOLD, 255, cv2.THRESH_BINARY)
    
    # Maske anwenden: Wo sie weiß ist, wird Weiß erzwungen. Sonst bleiben die Grauwerte.
    filtered = cv2.bitwise_or(denoised, ghosting_mask)
    
    # 5. Lokaler Kontrast-Boost via CLAHE
    clahe = cv2.createCLAHE(clipLimit=1.5, tileGridSize=(8, 8))
    enhanced = clahe.apply(filtered)
    
    print(f"[DEBUG] Ghosting-Filter angewendet (Threshold: {WHITE_THRESHOLD})")
    return cv2.cvtColor(enhanced, cv2.COLOR_GRAY2RGB)

def crop_to_receipt(img_np: np.ndarray) -> np.ndarray:
    """Sucht den hellen Kassenbon auf dunklem Grund und schneidet ihn aus."""
    print("[DEBUG] Analysiere Bildkonturen für Auto-Crop...")
    h, w = img_np.shape[:2]
    scale = 500 / max(h, w)
    small = cv2.resize(img_np, (int(w * scale), int(h * scale)), interpolation=cv2.INTER_AREA)
    gray = cv2.cvtColor(small, cv2.COLOR_RGB2GRAY)
    
    blurred = cv2.GaussianBlur(gray, (5, 5), 0)
    _, thresh = cv2.threshold(blurred, 0, 255, cv2.THRESH_BINARY + cv2.THRESH_OTSU)
    
    contours, _ = cv2.findContours(thresh, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    
    if contours:
        max_cnt = max(contours, key=cv2.contourArea)
        if cv2.contourArea(max_cnt) > (small.shape[0] * small.shape[1] * 0.08):
            print(f"[DEBUG] Passende Kontur gefunden (Fläche: {cv2.contourArea(max_cnt):.0f}px). Schneide zu...")
            rect = cv2.minAreaRect(max_cnt)
            
            # Fehlerbehebung: np.int32 sichert Kompatibilität in neueren OpenCV-Versionen
            box = np.int32(cv2.boxPoints(rect) / scale)
            
            pts = box.reshape(4, 2)
            rect_pts = np.zeros((4, 2), dtype="float32")
            s = pts.sum(axis=1)
            rect_pts[0] = pts[np.argmin(s)]
            rect_pts[2] = pts[np.argmax(s)]
            diff = np.diff(pts, axis=1)
            rect_pts[1] = pts[np.argmin(diff)]
            rect_pts[3] = pts[np.argmax(diff)]
            
            (tl, tr, br, bl) = rect_pts
            w_a = np.sqrt(((br[0] - bl[0]) ** 2) + ((br[1] - bl[1]) ** 2))
            w_b = np.sqrt(((tr[0] - tl[0]) ** 2) + ((tr[1] - tl[1]) ** 2))
            max_w = max(int(w_a), int(w_b))
            
            h_a = np.sqrt(((tr[0] - br[0]) ** 2) + ((tr[1] - br[1]) ** 2))
            h_b = np.sqrt(((tl[0] - bl[0]) ** 2) + ((tl[1] - bl[1]) ** 2))
            max_h = max(int(h_a), int(h_b))
            
            dst = np.array([[0, 0], [max_w - 1, 0], [max_w - 1, max_h - 1], [0, max_h - 1]], dtype="float32")
            m = cv2.getPerspectiveTransform(rect_pts, dst)
            return cv2.warpPerspective(img_np, m, (max_w, max_h))
            
    print("[DEBUG] Keine eindeutige Bon-Kontur erkannt. Nutze Originalmaße.")
    return img_np

def process_image_pipeline(image_path: str) -> str:
    print(f"\n--- Starte Pipeline für: {image_path} ---")
    try:
        pil_img = Image.open(image_path)
        pil_img = ImageOps.exif_transpose(pil_img)
        img_np = np.array(pil_img.convert("RGB"))
        print(f"[DEBUG] Bild erfolgreich geladen. Originalgröße: {img_np.shape[1]}x{img_np.shape[0]}")
    except Exception as e:
        print(f"[ERROR] Bildfehler bei {image_path}: {e}")
        return None

    # 1. Schritt: Ränder wegschneiden & gerade ausrichten
    cropped_np = crop_to_receipt(img_np)
    
    # 2. Schritt: Textqualität maximieren (Inklusive Ghosting-Filter)
    enhanced_np = maximize_text_quality(cropped_np)
    
    # 3. Schritt: Downscaling auf VLM Sweetspot (1024px)
    h, w = enhanced_np.shape[:2]
    max_side = 680
    if max(h, w) > max_side:
        scale_vlm = max_side / max(h, w)
        enhanced_np = cv2.resize(enhanced_np, (int(w * scale_vlm), int(h * scale_vlm)), interpolation=cv2.INTER_AREA)
        print(f"[DEBUG] Bild herunterskaliert auf: {enhanced_np.shape[1]}x{enhanced_np.shape[0]}")

    # 4. Schritt: Ausgabe des preprocessed Images nach debug_images
    os.makedirs(DEBUG_DIR, exist_ok=True)
    base_name = os.path.splitext(os.path.basename(image_path))[0]
    debug_out_path = os.path.join(DEBUG_DIR, f"prep_{base_name}.jpg")
    
    # OpenCV nutzt BGR für die Dateiablage, daher vor dem Schreiben konvertieren
    cv2.imwrite(debug_out_path, cv2.cvtColor(enhanced_np, cv2.COLOR_RGB2BGR))
    print(f"[DEBUG] Preprocessed Image gespeichert unter: {debug_out_path}")

    # Zu Base64 konvertieren (JPEG komprimiert mit hoher Qualität für das VLM)
    _, buffer = cv2.imencode('.jpg', cv2.cvtColor(enhanced_np, cv2.COLOR_RGB2BGR), [int(cv2.IMWRITE_JPEG_QUALITY), 90])
    return base64.b64encode(buffer).decode("utf-8")

# ---------------------------
# VLM INFERENCE
# ---------------------------
def run_vlm_analysis(b64_image: str):
    print(f"[DEBUG] Sende Daten an Ollama Model '{OLLAMA_MODEL}'...")
    
    prompt = """Extrahiere die Daten dieses deutschen Kassenbons präzise in das vorgegebene JSON-Format.
    
Erwartetes JSON-Schema:
{
  "haendler": "Name des Geschäfts / Kette",
  "artikel": [
    {"name": "Sauberer Artikelname", "menge": 1, "preis_einzel": 0.00, "preis_gesamt": 0.00}
  ],
  "gesamtbetrag": 0.00
}"""

    response = ollama.chat(
        model=OLLAMA_MODEL,
        messages=[
            {
                "role": "system",
                "content": "Du bist ein präziser OCR-Parser für deutsche Kassenbons. Antworte ausschließlich im geforderten JSON-Format."
            },
            {
                "role": "user",
                "content": prompt,
                "images": [b64_image]
            }
        ],
        options={
            "temperature": 0.0,
            "top_p": 0.1,
            "num_thread": 8,
        },
        format="json"
    )
    return response["message"]["content"]

# ---------------------------
# MAIN / CLI HANDLING
# ---------------------------
if __name__ == "__main__":
    # Unterstützung für Wildcards (*.jpg) und Einzeldateien über Argumente
    if len(sys.argv) < 2:
        print("Benutzung: python ocr.py <pfad_zum_bild_oder_wildcard>")
        print("Beispiel:  python ocr.py ./IMG_1196.jpeg")
        print("Beispiel:  python ocr.py './recepts/*.jpg'")
        sys.exit(1)

    input_arg = sys.argv[1]
    
    # glob auflösen für den Fall, dass Wildcards übergeben wurden
    file_list = glob.glob(input_arg)
    
    if not file_list:
        print(f"[ERROR] Keine Dateien unter '{input_arg}' gefunden.")
        sys.exit(1)
        
    print(f"[DEBUG] Gefundene Datei(en) zur Verarbeitung: {len(file_list)}")

    for file_path in file_list:
        if os.path.isdir(file_path):
            continue
            
        # Pipeline ausführen
        b64_img = process_image_pipeline(file_path)
        
        if b64_img is None:
            continue
            
        print("[DEBUG] VLM Inferenz gestartet (kann auf CPU einen Moment dauern)...")
        try:
            raw_result = run_vlm_analysis(b64_img)
            final_json = json.loads(raw_result.strip())
            
            print(f"\n✅ ERGEBNIS FÜR {file_path}:")
            print(json.dumps(final_json, indent=2, ensure_ascii=False))
            print("-" * 40)
            
        except Exception as e:
            print(f"\n❌ Fehler bei VLM/JSON-Parsing für {file_path}: {e}")
