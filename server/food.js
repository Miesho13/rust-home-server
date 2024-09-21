$(document).ready(function() {
    main();
});

const foodCtx = {
    dishDB: [],
    dishPick: [],

}

async function main() {
    try {
        init();

        setupEventListeners();
    }
    catch (err) {
        console.log("An error occured:", err);
    }

//    const data = await getJson("json_db/foods.json");
//    foodCtx.dishDB = data.foods;
//
//    console.log(foodCtx.dishDB);
//    console.log(findDishByName(foodCtx.dishDB, "Tibijka"));
//
//    $.each(foodCtx.dishDB, function(index, item) {
//        $('#sortable1').append('<li class="ui-state-default">' + item.name + '</li>');
//    });
//
//    $("#sortable1, #sortable2").sortable({
//        connectWith: ".connectedSortable"
//    }).disableSelection();
}

async function init() {
    foodCtx.dishDB = await getJson("json_db/foods.json");
    console.log(foodCtx.dishDB.foods);
}

function setupEventListeners() {
    // event for input changes
    $("#searchFood").on("input", handleInputChange)
    

}

function handleInputChange(event) {
    console.log("Input change:", event.target.value);
}

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

function findDishByName(dish, name) {
    return dish.find(d => d.name === name);
}

