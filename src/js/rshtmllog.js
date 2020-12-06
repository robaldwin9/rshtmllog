warnFiltered = false;
errorFiltered = false;
debugFiltered = false;
infoFiltered = false;



function filter(button){
	switch(button.className)
	{
		case "warn":
			hideElements("warn", button, warnFiltered);
			warnFiltered = !warnFiltered;
			break;
		case "error":
			hideElements("error", button, errorFiltered);
			errorFiltered = !errorFiltered;
			break;
		case "debug":
			hideElements("debug", button, debugFiltered);
			debugFiltered = !debugFiltered;
			break;
		case "info":
			hideElements("info", button, infoFiltered);
			infoFiltered = !infoFiltered;
			console.log("info filter");
			break;
		default:
			console.log("Defuault cause break");
			break;
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