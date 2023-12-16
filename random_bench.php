<?php

function main() {
    $rewards = [1, 2, 3, 4];
    
    $length = count($rewards);
    
    $totalPercent = array_sum($rewards);
    
    $countById = array_fill(0, count($rewards), 0);
    
    $runCount = 100000000;
    
    $start = microtime(true);

    for ($j = 0; $j < $runCount; $j++) {
        $random = mt_rand() / mt_getrandmax() * $totalPercent;
        for ($i = 0; $i < $length; $i++) {
            $random -= $rewards[$i];
            if ($random <= 0) {
                $countById[$i]++;
                break;
            }
        }
    }

    $elapsed = microtime(true) - $start;

    printf(
        "Total run: %s, elapsed: %.2fs. %.2f ns/run, %s runs/s\n",
        number_format($runCount),
        $elapsed,
        $elapsed / $runCount * 1e9,
        number_format(floor($runCount / $elapsed))
    );

    foreach ($countById as $i => $count) {
        printf("id: %d, count: %d (%.2f%%)\n", $i, $count, number_format(($count / $runCount) * 100, 6));
    }
}

main();
?>
