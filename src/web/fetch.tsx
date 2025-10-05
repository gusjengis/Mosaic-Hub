export async function loadLogs() {
    let url = "http://35.155.119.40";
    // let url = "localhost:8088";
    let endpoint = "logLoad";

    try {
        const byteArray = await fetchByteArray(url, endpoint);
	window.wasmBindings.receive_body(byteArray);

    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

export async function fetchByteArray(url: string, endpoint: string): Promise<Uint8Array> {
    try {
        const response = await fetch(`${url}/${endpoint}`, {
            method: 'GET',
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const arrayBuffer = await response.arrayBuffer();
        return new Uint8Array(arrayBuffer);
    } catch (error) {
        console.error('Error fetching byte array:', error);
        throw error;
    }
}

