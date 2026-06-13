self.onmessage = async (e) => {
    const { file, url } = e.data;

    try {
        self.postMessage({ status: 'uploading' });

        const res = await fetch(url, { method: 'POST', headers: { 'Content-Type': file.type }, body: file });

        if (!res.ok) throw new Error(`Server error (${res.status})`);

        self.postMessage({ status: 'processing' });

        const payload = await res.json();

        self.postMessage({ payload });
    } catch (err) {
        self.postMessage({ error: err.message });
    }
};
