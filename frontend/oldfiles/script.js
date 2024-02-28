const cidList = document.querySelectorAll("ol.cidList input.field");
console.log(cidList.length);
for (i=0; i<cidList.length; i++){
    cidList[i].addEventListener("keydown", cidMax);
}

function cidMax(event){
    switch (event.key){
        case "Subtract":
        case " ":
            event.preventDefault();
            return;
        case "Tab":
            return;
        case "Enter":
        case "ArrowRight":
            let currentIndex = 0;
            for (i=0; i<cidList.length; i++){
                if (event.target == cidList[i]){
                    currentIndex = i;
                    break;
                }
            }
            if (currentIndex == cidList.length-1){
                event.preventDefault();
                return;
            }
            cidList[++currentIndex].focus();
            return;
        case "Backspace":
        case "ArrowLeft":
            if (event.target.value.length != 0 && event.key == "Backspace"){
                return;
            }
            let currentIndex1 = 0;
            for (i=0; i<cidList.length; i++){
                if (event.target == cidList[i]){
                    currentIndex1 = i;
                    break;
                }
            }
            if (currentIndex1 == 0){
                event.preventDefault();
                return;
            }
            cidList[--currentIndex1].focus();
            return;
    }
    if (isNaN(event.key)){event.preventDefault(); return;}
    if (event.target.value.length == Number(event.target.getAttribute("maxlength"))){
        let currentIndex = 0;
        let unfilledFields = [];
        for (i=0; i<cidList.length; i++){
            if (cidList[i].value.length < Number(cidList[i].getAttribute("maxlength"))){
                unfilledFields.push(i);
            }
            if (event.target == cidList[i]){
                currentIndex = i;
            }
        }
        if (unfilledFields.length == 0){
            event.preventDefault();
            event.target.value = event.key;
            if (currentIndex == cidList.length-1){
                return;
            }
            cidList[++currentIndex].focus();
            return;
        }
        let diff = Math.abs(unfilledFields[0]-currentIndex);
        let target = currentIndex;
        for (i=0; i<unfilledFields.length;i++){
            if (diff >= Math.abs(unfilledFields[i]-currentIndex)){
                diff = Math.abs(unfilledFields[i]-currentIndex);
                target = unfilledFields[i];
            }
        }
        console.log("Diff: " + diff + "\nTarget: " + target);
        cidList[target].focus();
    }
}
