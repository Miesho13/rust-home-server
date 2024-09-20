$(document).ready(function() {
    let itemCount = 0;

    $('#additem').click(function() {
        itemCount++;
        const newItem = $('<div></div>').addClass('listItem').text('Item ' + itemCount);
        $('#listContainer').append(newItem);
    });
});



const pathToFood = "json_db/foods.json"
async function getJson() {
    try {
        const respone = await fetch(pathToFood);

        if (!respone.ok) {
            throw new Error("Network respone was not ok" + respone.statusText);
        }
        
        const data = await respone.json();
        console.log(data);
    }
    catch(error) {
        console.error("Theres some problem.", error);
    }
}

getJson();




