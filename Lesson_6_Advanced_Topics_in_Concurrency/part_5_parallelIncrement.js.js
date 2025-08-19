let sharedState = 0;

async function incrementState() {
    const temp = sharedState;
    await new Promise(resolve => setTimeout(resolve, 1000));
    sharedState = temp + 1;
}

async function runInParallel() {
    await Promise.all([incrementState(), incrementState(), incrementState()]);
    console.log(`Final shared state: ${sharedState}`);
}

runInParallel();
