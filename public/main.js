async function init() {
  let rustApp = null;

  try {
    rustApp = await import("../pkg");
  } catch (e) {
    console.error(e);
    return;
  }
  console.log(rustApp);

  // <input type="file"/>
  const input = document.getElementById("upload");

  // refer to node.js docs
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    // chain on the replace method to remove metadata
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    const img_data_url = rustApp.grayscale(base64);
    document.getElementById("new-img").setAttribute("src", img_data_url);
  };
  // readAsDataURL stringifies to make it easier to send to rust
  input.addEventListener("change", () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
