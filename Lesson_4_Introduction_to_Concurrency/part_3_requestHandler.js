function requestHandler(url) {
    return new Promise((resolve) => {
        setTimeout(() => {
            console.log(`Handled request for ${url}`);
            resolve(`Response from ${url}`);
        }, 1000); // Simulate delay
    });
}

async function handleRequests() {
    console.log("Handling requests...");
    const response1 = await requestHandler("http://example.com/1");
    const response2 = await requestHandler("http://example.com/2");
    console.log(response1);
    console.log(response2);
}

handleRequests();
