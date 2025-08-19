async function throwError() {
    throw new Error("Something went wrong!");
}

async function main() {
    try {
        await throwError();
    } catch (error) {
        console.error(error.message);
    }
}

main();
