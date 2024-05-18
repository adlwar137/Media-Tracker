function getCookie(cookieName) {
		if(document.cookie == "") {
				return null;
		}

		cookieValue = document.cookie
				.split("; ")
				.find((row) => row.startsWith(cookieName))
				?.split("=")[1];
		deserialized  = JSON.parse(cookieValue);
		return deserialized;
}


function saveCookie(cookieName, cookieValue) {
		const settings = "; SameSite=Lax; max-age=315360000;";
		document.cookie = cookieName + "=" + JSON.stringify(cookieValue) + settings;
}

function generateTable(table) {
		if(document.getElementById("table") != null) {
				document.getElementById("table").remove();
		}

		if(table == null) {
				return;
		}

		const tbl = document.createElement("table");
		const tblBody = document.createElement("tbody");

		// create all cells
		for (let j = 0; j < table.length; j++) {
				//creates a table row
				const row = document.createElement("tr");

				for (let k = 0; k < table[j].length; k++) {
						const cell = document.createElement("td");
						const cellText = document.createTextNode(table[j][k]);
						cell.appendChild(cellText);
						row.appendChild(cell);
				}

				tblBody.appendChild(row);
		}
		tbl.appendChild(tblBody);
		document.getElementById("tableContainer").appendChild(tbl);
		tbl.setAttribute("id", "table");
		tbl.setAttribute("border", "2");
}

function addEntry() {
		const newEntry = document.getElementById("entryBox").value;
		const entryState = document.getElementById("entryStateBox").value;

		//update cookie
		var entryTable = getCookie("table");
		if(entryTable == null) {
				entryTable = [[newEntry, entryState]]
		} else {
				entryTable.push([newEntry, entryState]);
		}
		saveCookie("table", entryTable);

		generateTable(entryTable);
}

function modifyEntry() {
		const entry = document.getElementById("entryBox").value;
		const entryState = document.getElementById("entryStateBox").value;

		//retrieve cookie
		var entryTable = getCookie("table");

		//modify cookie
		for(let i = 0; i < entryTable.length; i++) {
				if(entryTable[i][0] == entry) {
						entryTable[i][1] = entryState;
				}
		}

		//save cookie
		saveCookie("table", entryTable);

		//update view
		generateTable(entryTable);
}

function removeEntry() {
		const entry = document.getElementById("entryBox").value;
		
		//retrieve cookie
		var entryTable = getCookie("table");

		//modify cookie
		temp = entryTable;
		for(let i = 0; i < entryTable.length; i++) {
				if(entryTable[i][0] == entry) {
						temp.splice(i,1);
				}
		}

		entryTable = temp;
		//save cookie
		saveCookie("table", entryTable);

		//update view
		generateTable(entryTable);
}

