// js/index.js
import('../pkg/index.js').then(module => {
    const { search_kanji_by_stroke_count } = module;

    document.getElementById('inputNumber').addEventListener('input', function () {
        const inputNumber = document.getElementById('inputNumber').valueAsNumber;
        const result = search_kanji_by_stroke_count(inputNumber);
        
        if (result.size > 0) {
            // Loop through the result and display the kanji in a new div which contains an H1 for the key, and a p tag for the value
            const resultDiv = document.getElementById('resultDiv');
            const howManyF = document.getElementById('kanji');
            resultDiv.innerHTML = '';
            howManyF.innerHTML = result.size + " kanji found";
            
            let counter = 0;  // Counter to keep track of divs in a row
            let currentRow;   // Variable to hold the current row div
            
            result.forEach((value, key) => {
                if (counter % 4 === 0) {
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
                meaning.textContent = "Meaning: " + (value.meanings.length == 0 ? "No meaning found" : value.meanings.join(', '));
                onP.textContent = "On reading: " + (value.readings_on.length == 0 ? "No on reading found" : value.readings_on.join(' and  '));
                kunP.textContent = "Kun reading: " + (value.readings_kun.length == 0 ? "No kun reading found" : value.readings_kun.join(' and '));

                newDiv.appendChild(newH1);
                newDiv.appendChild(newP);
                newDiv.appendChild(meaning);
                newDiv.appendChild(onP);
                newDiv.appendChild(kunP);

                currentRow.appendChild(newDiv);
                counter++;
            });
        }else if (inputNumber && result.size === 0) {
            const howManyF = document.getElementById('kanji');
            howManyF.innerHTML = result.size + " kanji found";
            const resultDiv = document.getElementById('resultDiv');
            resultDiv.innerHTML = '';
        } 
        else {
            const resultDiv = document.getElementById('resultDiv');
            resultDiv.innerHTML = '';
            const howManyF = document.getElementById('kanji');
            howManyF.innerHTML = 'Enter a number';
        }
    });

    document.getElementById('rustButton').addEventListener('click', function () {
        const inputNumber = document.getElementById('inputNumber').valueAsNumber;
        const result = search_kanji_by_stroke_count(inputNumber, data);
    });
});
