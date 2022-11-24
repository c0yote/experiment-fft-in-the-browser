const appendNumberToBody = (number) => {
    const text = document.createTextNode(number);
    document.body.append(text);
}

const alert2 = () => {
    alert("alert2");
}

let application = null;

const init_shim = (app) => {
    application = app;
}

export { init_shim, alert2, appendNumberToBody };