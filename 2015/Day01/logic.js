exports.FinalFloor = function FinalFloor( Input, currentFloor = 0 ) {
    var CurrentFloor = currentFloor;

    var ups = Input.match(/\(/g).length;
    var dns = Input.match(/\)/g).length;

    CurrentFloor += ups - dns;
    return CurrentFloor;
}

exports.FirstTimeAtFloor = function FirstTimeAtFloor(Input, desiredFloor){
    var currentFloor = 0;
    for(var i = 0; i < Input.length; i++){
        currentFloor += this.SymbolValue(Input.charAt(i));
        if(currentFloor === desiredFloor){
            return i + 1;
        }
    }

    return -1;
}

exports.SymbolValue = function SymbolValue( character ){
    if(character === ')'){
        return -1;
    }
    else if( character === '('){
        return 1;
    }
    else {
        return 0;
    }
}