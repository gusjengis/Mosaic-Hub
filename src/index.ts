window.onload = async function() {
	let output = document.getElementById("output");
	var logs = "";
	await fetchAllLogs().then(res => logs = res).catch(error => logs = "Error Fetching Logs: " + error.toString());
	if (output) {
		output.innerText = logs;
	}
}
