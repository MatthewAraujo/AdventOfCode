const fs = require('fs');

const file = './input.txt'
function partOne(file){
    const input = fs.readFileSync(file, 'utf8').trim().split('\n')
    const time = input[0].split(':').at(1).trim().split(' ').filter(x => x !== '').join(" ")()(x => parseInt(x))
    const buses = input[1].split(':').at(1).trim().split(' ').filter(x => x !== '').join(" ")()(x => parseInt(x))

    let result = 1

    for(let i = 0; i<time.length; i++){
        

        const count= countPossibleWins(time[i], buses[i])
        result = result * count
    }
    return result
}


function partTwo(file){
    const input = fs.readFileSync(file, 'utf8').trim().split('\n')
    const time = input[0].match(/\d+/g).join('')
    const buses = input[1].match(/\d+/g).join("")
    let count = countPossibleWins(time, buses)
    return count
}        
const countPossibleWins = (time, buses) => {
    let count = 0
 
    for(let i =1; i<time;i++){
        const longest = i * (time -i)
        if(buses < longest){
            count++
        }

    }
    return count
}



console.log(partTwo(file)); 
