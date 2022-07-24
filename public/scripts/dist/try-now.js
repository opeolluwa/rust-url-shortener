"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
//reference the elements in the DOM
const tryNowForm = document.querySelector("#try-now-form");
const inputFeed = document.querySelector("#try-now-form input");
const tryNowButton = document.querySelector("#try-now-form button");
//add event listener to the button
tryNowForm === null || tryNowForm === void 0 ? void 0 : tryNowForm.addEventListener("submit", function (event) {
    return __awaiter(this, void 0, void 0, function* () {
        event.preventDefault(); //prevent auto submission
        const inputValue = inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.value; //get the value of the input
        const validInput = validateInput(inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.value);
        if (!validInput) {
            appendStyle("d-inline-block");
        }
        tryNowButton.disabled = true; //disable the button
        tryNowButton.innerHTML = "Loading..."; //change the button text
        const serverResponse = yield postData("/api/v1/links/minify", {
            original_url: inputValue
        });
        const { success, message, data } = yield serverResponse;
        //det the button to active 
        tryNowButton.disabled = false;
        tryNowButton.innerHTML = "Try Now 🚀";
        //clear the input field
        if (!success) {
            const pasteBin = document.querySelector("#try-now-form span.error-message");
            pasteBin.innerText = message;
            tryNowButton.addEventListener("click", clipBoard);
            return;
        }
        //if error 
        inputFeed.value = data;
        inputFeed.select();
        inputFeed.setSelectionRange(0, 99999);
        navigator.clipboard.writeText(inputFeed.value);
        tryNowButton.innerHTML = "Copy 📋";
    });
});
//hide the error message on innput focus{
inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.addEventListener("input", function () {
    // document?.querySelector("#try-now-form span.error-message")?.classList.remove("show-error")
    removeStyle("d-inline-block");
});
inputFeed === null || inputFeed === void 0 ? void 0 : inputFeed.addEventListener("focus", function () {
    // document?.querySelector("#try-now-form span.error-message")?.classList.remove("show-error")
    removeStyle("d-inline-block");
});
//if no input or invalid input
function validateInput(inputValue) {
    if ( /* !inputValue.trim().startsWith("http://") || */!inputValue.trim().startsWith("http")) {
        return false;
    }
    return true;
}
function removeStyle(className, domSelector = "#try-now-form span.error-message") {
    var _a;
    (_a = document === null || document === void 0 ? void 0 : document.querySelector(domSelector)) === null || _a === void 0 ? void 0 : _a.classList.remove(className);
}
function appendStyle(className, domSelector = "#try-now-form span.error-message") {
    var _a;
    (_a = document === null || document === void 0 ? void 0 : document.querySelector(domSelector)) === null || _a === void 0 ? void 0 : _a.classList.add(className);
}
// Example POST method implementation:
function postData(url = '', data = {}) {
    return __awaiter(this, void 0, void 0, function* () {
        // Default options are marked with *
        const response = yield fetch(url, {
            method: 'POST',
            mode: 'cors',
            cache: 'no-cache',
            credentials: 'same-origin',
            headers: {
                'Content-Type': 'application/json'
                // 'Content-Type': 'application/x-www-form-urlencoded',
            },
            redirect: 'follow',
            referrerPolicy: 'no-referrer',
            body: JSON.stringify(data) // body data type must match "Content-Type" header
        });
        return response.json(); // parses JSON response into native JavaScript objects
    });
}
function clipBoard() {
    inputFeed.select();
    inputFeed.setSelectionRange(0, 99999);
    navigator.clipboard.writeText(inputFeed.value);
    // alert("Copied the text: " + inputFeed.value);
    inputFeed.value = "";
    tryNowButton.innerHTML = "Try Now 🚀";
}
