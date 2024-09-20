const foodCtx = {
    dishDB: [],
    dishPick: [],
}

async function main() {
    foodCtx.dishDB = await getJson("json_db/foods.json");
    console.log(foodCtx.dishDB);
    console.log(foodCtx.dishDB.find(dish => dish.name === "Tibijka"));
}

$(document).ready(function() {
    let itemCount = 0;
    $('#additem').click(function() {
        itemCount++;
        const newItem = $('<div></div>').addClass('listItem').text('Item ' + itemCount);
        $('#listContainer').append(newItem);
    });
});

async function getJson(pathToFood) {
    let respone;
    try {
        respone = await fetch(pathToFood);

        if (!respone.ok) {
            throw new Error("Network respone was not ok" + respone.statusText);
        }
    }
    catch(error) {
        console.error("Theres some problem.", error);
    }

    return respone.json(); 
}

main();
