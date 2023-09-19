async function init(){
	let rustApp = null;

	try{
		rustApp = await import('../pkg');
	}catch(e){
		console.error(`Unexpected error in load rust app: ${e}`);
		return;
	}

	
	console.log(rustApp);

	const input = document.getElementById('upload');
	const fileReader = new FileReader();

	fileReader.onloadend = (e) => {
		const base64String = fileReader.result.replace(/^data:image\/(png|jpg|jpeg);base64,/, "");

		let img_data_url = rustApp.grayscale(base64String);

		document.getElementById('new-img').setAttribute('src', img_data_url);
	};

	input.addEventListener('change', (e) => {
		fileReader.readAsDataURL(input.files[0]);
	});
}

init()