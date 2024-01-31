// js/index.js
import('../pkg/index.js').then(module => {
    const { add } = module;
    
    document.getElementById('button').addEventListener('click', function () {
        const inputNumber = document.getElementById('inputNumber').valueAsNumber;
        const result = add(inputNumber, 3); 
        document.getElementById('result').textContent = `Result: ${result}`;
    });
});
