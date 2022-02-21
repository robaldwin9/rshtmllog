warnFiltered = false;
errorFiltered = false;
debugFiltered = false;
infoFiltered = false;

function filter(button){
	switch(button.className)
	{
		case "WARN":
			new Worker(hideElements("warn", button, warnFiltered));
			warnFiltered = !warnFiltered;
			break;
		case "ERROR":
			new Worker(hideElements("error", button, errorFiltered));
			errorFiltered = !errorFiltered;
			break;
		case "DEBUG":
			new Worker(hideElements("debug", button, debugFiltered));
			debugFiltered = !debugFiltered;
			break;
		case "INFO":
			new Worker(hideElements("info", button, infoFiltered))
			infoFiltered = !infoFiltered;
			break;
		case "TRACE":
			new Worker(hideElements("trace", button, infoFiltered))
			infoFiltered = !infoFiltered;
			break;
		default:
			console.log("Default cause break");
			break;
	}
}


function view(div) {

	if(div.style.height != "100%") {
		div.style.height = "100%"
		div.style.lineheight = "100%"

		
		div.style.wordWrap= "break-word"

		div.firstChild.style.wordWrap= "break-word"
	
		div.firstChild.style.whiteSpace = "normal"
		
		
		
	} else {
		div.style.height = "3vw"
		div.style.lineheight = "1.5vw "
		
		div.style.overflow  = "hidden"
		div.style.overflowwrap = "none"

		div.firstChild.style.overflow = "none"
		div.firstChild.style.overflow = "hidden"
		div.firstChild.style.whiteSpace = "nowrap"
	}
}



function hideElements(classname, buttonRef, filtered) {
	elements = document.getElementsByClassName(classname)

	for(i = 0; i < elements.length; i++)
	{
		element = elements[i];
		if(element !== buttonRef)
		{
			if(filtered) {
				element.style.display = "block";
			}

			else{
				element.style.display = "none";
			}
			
		}


		else
		{
					if(filtered) {
				element.style.borderColor = "white";
				
			}

			else{
				element.style.borderColor = "green";
			
			}
		}
	}
}
