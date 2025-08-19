function fetchData() {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            const data = "Data received!";
            // Uncomment the next line to simulate an error
            // reject("Error: Unable to fetch data");
            resolve(data);
        }, 3000); // Simulating a network request delay
    });
}

fetchData()
    .then(response => {
        console.log(response);
    })
    .catch(error => {
        console.error(error);
    });
