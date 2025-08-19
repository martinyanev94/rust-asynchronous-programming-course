async function fetchData() {
    console.log("Fetching data...");
    const data = await new Promise((resolve) => {
        setTimeout(() => {
            resolve("Data received from JavaScript!");
        }, 2000);
    });
    return data;
}

fetchData().then(result => console.log(result));
