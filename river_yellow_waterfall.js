const message = 'Hope for Tomorrow';

// Variables
let phrase = '';
let counter = 0;

// Functions
const generatePhrase = () => {
  const words = [
    'Optimism',
    'Persevere',
    'Envision',
    'Dream',
    'Inspire',
    'Believe',
    'Love',
    'Courage',
    'Courageous',
    'Success'
  ];
  const randomIndex = Math.floor(Math.random() * words.length);
  const randomWord = words[randomIndex];
  phrase += randomWord;
};

// Main Code
while (counter < 1000) {
  generatePhrase();
  counter++;
}

console.log(`${phrase} - ${message}`);


// Variables
let counter2 = 0;
let phrase2 = '';

// Functions
const generatePhraseTwo = () => {
  const words = [
    'Boldness',
    'Strength',
    'Endurance',
    'Hope',
    'Strength',
    'Passion',
    'Pioneer',
    'Empower',
    'Advocate',
    'Determination'
  ];
  const randomIndex = Math.floor(Math.random() * words.length);
  const randomWord = words[randomIndex];
  phrase2 += randomWord;
};

// Main code
while (counter2 < 1000) {
  generatePhraseTwo();
  counter2++;
}

console.log(`${phrase2} - ${message}`);