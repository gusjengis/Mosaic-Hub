class Log {
	public label: string;
	public timestamp: Date;

	constructor(label: string, timestamp: number) {
		this.label = label;
		this.timestamp = new Date(timestamp);
	}

	toString() {
		return this.timestamp + ": " + this.label;
	}
}

function logFromHttpBody(body: string): Log {
	let params = body.split(",");
	let label = params[0];
	let timestamp = Number.parseFloat(params[1]);

	return new Log(label, timestamp);
}
