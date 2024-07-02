ps | find surreal | get name pid
 each {|process| print $"Name: ($process.name), pid: ($process.pid)" } 

ps | find surreal | get name pid | each {|process| let name = $process.0; let pid = $process.1; print $"Name: ($name), pid: ($name)" }

ps | filter {|ps| $ps.name == "surreal"} | get pid | each {|pid| kill $pid }