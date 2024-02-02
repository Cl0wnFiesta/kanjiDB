// js/index.js
import data from '../data/kanjies.json';
import('../pkg/index.js').then(module => {
    const { add } = module;

    document.getElementById('button').addEventListener('click', function () {
        const inputNumber = document.getElementById('inputNumber').valueAsNumber;
        const result = add(inputNumber, 3);
        document.getElementById('result').textContent = `Result: ${result}`;
    });
});
const dataArray = Object.values(data);

import('../pkg/index.js').then(module => {
    const { search_kanji_by_stroke_count } = module;
    console.log(dataArray);

    document.getElementById('inputNumber').addEventListener('input', function () {
        const inputNumber = document.getElementById('inputNumber').valueAsNumber;
        const result = search_kanji_by_stroke_count(inputNumber, data);
        
        if (result.size > 0) {
            document.getElementById('result').textContent = `Result: ${result}`;
            
            // Loop through the result and display the kanji in a new div which contains an H1 for the key, and a p tag for the value
            const resultDiv = document.getElementById('resultDiv');
            resultDiv.innerHTML = '';
            
            let counter = 0;  // Counter to keep track of divs in a row
            let currentRow;   // Variable to hold the current row div
            
            result.forEach((value, key) => {
                if (counter % 4 === 0) {
                    console.log("apina");
                    // Create a new row div for every 4 divs
                    currentRow = document.createElement('div');
                    currentRow.classList.add('row');
                    resultDiv.appendChild(currentRow);
                }

                const newDiv = document.createElement('div');
                newDiv.classList.add('result-item');  // Apply a class for styling
                const newH1 = document.createElement('h1');
                const newP = document.createElement('p');
                const meaning = document.createElement('p');
                const onP = document.createElement('p');
                const kunP = document.createElement('p');

                newH1.textContent = key;
                newP.textContent = "Stroke: " + value.strokes;
                meaning.textContent = "Meaning: " + value.meanings.join(', ');
                onP.textContent = "On reading: " + value.readings_on.join(', ');
                kunP.textContent = "Kun reading: " + value.readings_kun.join(', ');

                newDiv.appendChild(newH1);
                newDiv.appendChild(newP);
                newDiv.appendChild(meaning);
                newDiv.appendChild(onP);
                newDiv.appendChild(kunP);

                currentRow.appendChild(newDiv);
                counter++;
            });
        } else {
            document.getElementById('result').textContent = `Result: No result`;

            const resultDiv = document.getElementById('resultDiv');
            resultDiv.innerHTML = '';
        }
    });

    document.getElementById('rustButton').addEventListener('click', function () {
        const inputNumber = document.getElementById('inputNumber').valueAsNumber;
        const result = search_kanji_by_stroke_count(inputNumber, data);
        console.log(result);
    });
});