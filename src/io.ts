async function fetchAllLogs(): Promise<string> {
	const response = await fetch('http://35.155.119.40:80/logLoad');
	if (!response.ok) {
		throw new Error(`HTTP error! status: ${response.status}`);
	}
	const logs = await response.text();
	return logs;
}
