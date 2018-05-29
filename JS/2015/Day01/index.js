var logic = require('./logic');
var { input } = require ('./input');

var partOneAns = logic.FinalFloor(input, 0);

console.log("PART ONE ANSWER: " + partOneAns);

var partTwoAns = logic.FirstTimeAtFloor(input, -1);

console.log("PART TWO ANSWER: " + partTwoAns);
