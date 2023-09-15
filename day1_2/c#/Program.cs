var log = (object msg) => Console.WriteLine(msg);

string ReadInputTxt() {
    var contents = File.ReadAllText("../input.txt");
    if (contents is null) {
        throw new FileNotFoundException("Error reading input file");
    }
    return contents;
}

var input = ReadInputTxt();
var elves = input.Trim().Split("\n\n");
var cals = new List<int>();
foreach (var elf in elves) {
    var elfCals = 0;
    foreach (var foodItem in elf.Trim().Split("\n")) {
        var foodCals = int.Parse(foodItem);
        elfCals += foodCals;
    }
    cals.Add(elfCals);
}
cals.Sort();

var top3Cals = cals.TakeLast(3).Sum();
log(top3Cals);
