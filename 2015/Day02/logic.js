exports.boxes = [];

exports.ParseEntries = function ParseEntries(input){
    var stringArr = input.split('\n');
    for(var i = 0; i < stringArr.length; i++){
        var dimensionsStr = stringArr[i].split('x');
        var dimensions = [];
        for(var j = 0; j < dimensionsStr.length; j++){
            dimensions[j] = parseInt(dimensionsStr[j]);
        }
        dimensions.sort();
        this.boxes.push(dimensions);
    }
}

exports.GetTotalSurfaceArea = function GetTotalSurfaceArea(input){
    this.ParseEntries(input);

    var accum = 0;
    for(var i = 0; i < this.boxes.length; i++){
        var sides = this.GetSurfaceAreaOfSides(this.boxes[i][0], this.boxes[i][1], this.boxes[i][2]);
        sides.sort();
        
        accum += (3 * sides[0]) + (2 * sides[1]) + (2 * sides[2]);
    }

    return accum;
}

exports.GetSurfaceAreaOfSides = function GetSurfaceAreaOfSides(l, w, h){
    var sides = [
        (l * w),
        (w * h),
        (h * l)
    ]

    return sides;
}