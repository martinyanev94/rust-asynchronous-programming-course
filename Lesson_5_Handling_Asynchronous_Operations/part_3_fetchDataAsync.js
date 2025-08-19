async function fetchDataAsync() {
    try {
        const response = await fetchData();
        console.log(response);
    } catch (error) {
        console.error(error);
    }
}

fetchDataAsync();
