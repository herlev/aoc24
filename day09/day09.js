import { readFileSync } from 'fs'

Array.prototype.sum = function() {
  return this.reduce((acc, val) => acc + val, 0)
}

Array.prototype.max = function() {
  return this.reduce((acc, val) => (val > acc ? val : acc), -Infinity)
}

Array.prototype.findAndIndex = function(callback) {
  for (let i = 0; i < this.length; i++) {
    if (callback(this[i], i, this)) {
      return [this[i], i]
    }
  }
  return [undefined, -1] // If no match is found
}

function suminRange(start, end) {
  return ((end - start + 1) / 2) * (start + end)
}

let input = "2333133121414131402"
input = readFileSync("input.txt").toString()

let currentIndex = 0
input = input.split("").map((value, index) => {
  let o = { count: value - 0, type: index % 2 == 0 ? "file" : "free space" }
  if (o.type == "file") {
    o.id = index / 2
  }
  o.offset = currentIndex
  currentIndex += o.count
  return o
})

function part1(i) {
  let input = JSON.parse(JSON.stringify(i)) // deep copy
  let s2 = []
  for (let current of input) {
    if (current.type == "file") {
      s2.push({ id: current.id, count: current.count, offset: current.offset })
      continue
    }
    // free space
    while (current.count != 0) {
      let file = input.pop()
      if (file.type == "free space" || file.count == 0) {
        continue
      }
      let numToGrab = Math.min(current.count, file.count)
      s2.push({ id: file.id, count: numToGrab, offset: current.offset })
      current.offset += numToGrab
      current.count -= numToGrab
      file.count -= numToGrab
      input.push(file)
    }
  }
  console.log(s2.map(({ id, count, offset }) => id * suminRange(offset, offset + count - 1)).sum())
}

function part2(i) {
  let input = JSON.parse(JSON.stringify(i)) // deep copy
  let maxId = input.filter(i => i.type == "file").map(i => i.id).max()
  for (let id = maxId; id >= 0; id--) {
    let [file, file_i] = input.findAndIndex(f => f.id == id)
    let [freeSpace, freeSpace_i] = input.findAndIndex(f => f.type == "free space" && f.offset < file.offset && f.count >= file.count)
    if (freeSpace) {
      input.splice(file_i, 1) // remove
      input.splice(freeSpace_i, 0, file)
      file.offset = freeSpace.offset
      freeSpace.offset += file.count
      freeSpace.count -= file.count
    }
  }
  console.log(input.filter(f => f.type == "file").map(({ id, count, offset }) => id * suminRange(offset, offset + count - 1)).sum())
}

part1(input)
part2(input)
