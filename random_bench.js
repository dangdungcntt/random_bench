function main() {
    const rewards = [1, 2, 3, 4];

    const length = rewards.length;

    const totalPercent = rewards.reduce((sum, reward) => sum + reward, 0);

    const countById = new Array(rewards.length).fill(0);

    const runCount = 100000000;

    const start = new Date();

    for (let j = 0; j < runCount; j++) {
        let random = Math.random() * totalPercent;
        for (let i = 0; i < length; i++) {
            random -= rewards[i];
            if (random <= 0) {
                countById[i]++;
                break;
            }
        }
    }

    const elapsed = new Date() - start;

    console.log(
        `Total run: ${numberWithCommas(runCount)}, elapsed: ${(elapsed / 1000).toFixed(2)}s. ${(elapsed / runCount * 1e6
        ).toFixed(2)} ns/run, ${numberWithCommas(Math.floor(
            (runCount / (elapsed / 1000))
        ))} runs/s`
    );

    countById.forEach((count, i) => {
        console.log(`id: ${i}, count: ${count} (${((count / runCount) * 100).toFixed(6)}%)`);
    });
}

function numberWithCommas(x) {
    return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

main();
