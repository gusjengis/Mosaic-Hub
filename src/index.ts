var logs = new Array<Log>();

window.onload = async function() {
	let output = document.getElementById("output");
	var logBody = "";
	await fetchAllLogs().then(res => logBody = res).catch(error => logBody = "Error Fetching Logs: " + error.toString());

	let splitBody = logBody.split("\n");

	for (const line of splitBody) {
		logs.push(logFromHttpBody(line));
	}

	var out = ""

	for (const log of logs) {
		out += log + "\n";
	}
	if (output) {
		output.innerText = out;
	}
}
