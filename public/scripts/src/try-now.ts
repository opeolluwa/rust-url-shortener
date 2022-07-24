//reference the elements in the DOM
const tryNowForm = document.querySelector("#try-now-form");
const inputFeed: any = document.querySelector("#try-now-form input");
const tryNowButton: any = document.querySelector("#try-now-form button");


//add event listener to the button
tryNowForm?.addEventListener("submit", async function (event) {
    event.preventDefault();//prevent auto submission
    const inputValue = inputFeed?.value; //get the value of the input
    const validInput: boolean = validateInput(inputFeed?.value)
    if (!validInput) {
        appendStyle("d-inline-block")
    }
    tryNowButton.disabled = true;//disable the button
    tryNowButton.innerHTML = "Loading...";//change the button text
    const serverResponse: any = await postData("/api/v1/links/minify", {
        original_url: inputValue
    });
    const { success, message, data } = await serverResponse;
    //det the button to active 
    tryNowButton.disabled = false;
    tryNowButton.innerHTML = "Try Now 🚀";
    //clear the input field
    if (!success) {
        const pasteBin = document.querySelector("#try-now-form span.error-message") as HTMLSpanElement;
        pasteBin.innerText = message;
        tryNowButton.addEventListener("click", clipBoard)
        return
    }
    //if error 
    inputFeed.value = data;
    inputFeed.select();
    inputFeed.setSelectionRange(0, 99999);
    navigator.clipboard.writeText(inputFeed.value);
    tryNowButton.innerHTML = "Copy 📋"

});

//hide the error message on innput focus{
inputFeed?.addEventListener("input", function () {
    // document?.querySelector("#try-now-form span.error-message")?.classList.remove("show-error")
    removeStyle("d-inline-block")
});

inputFeed?.addEventListener("focus", function () {
    // document?.querySelector("#try-now-form span.error-message")?.classList.remove("show-error")
    removeStyle("d-inline-block")
});

//if no input or invalid input
function validateInput(inputValue: string): boolean {
    if (/* !inputValue.trim().startsWith("http://") || */ !inputValue.trim().startsWith("http")) {
        return false
    }
    return true
}
function removeStyle(className: string, domSelector: string = "#try-now-form span.error-message",) {
    document?.querySelector(domSelector)?.classList.remove(className)
}
function appendStyle(className: string, domSelector: string = "#try-now-form span.error-message",) {
    document?.querySelector(domSelector)?.classList.add(className)
}


// Example POST method implementation:
async function postData(url = '', data: any = {}) {
    // Default options are marked with *
    const response = await fetch(url, {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
            // 'Content-Type': 'application/x-www-form-urlencoded',
        },
        redirect: 'follow', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        body: JSON.stringify(data) // body data type must match "Content-Type" header
    });
    return response.json(); // parses JSON response into native JavaScript objects
}



function clipBoard() {
    inputFeed.select();
    inputFeed.setSelectionRange(0, 99999);
    navigator.clipboard.writeText(inputFeed.value);
    // alert("Copied the text: " + inputFeed.value);
    inputFeed.value = "";
    tryNowButton.innerHTML = "Try Now 🚀";
}