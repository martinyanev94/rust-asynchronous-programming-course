const controller = new AbortController();
const signal = controller.signal;

async function fetchData(signal) {
    const response = await new Promise((resolve, reject) => {
        setTimeout(() => {
            if (signal.aborted) {
                reject(new Error('Fetch aborted'));
            } else {
                resolve("Data received");
            }
        }, 3000);
    });
    return response;
}

fetchData(signal).catch(error => console.error(error.message));

// Cancel the fetch after 1 second
setTimeout(() => {
    controller.abort(); 
    console.log("Fetch operation aborted.");
}, 1000);
