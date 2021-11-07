import createActor from "./actor";

createActor().consume_request({})
    .then(json => {
        if (json['data']) {
            console.log("Received request json", json['data']);
        } else {
            console.log("Error", json);
        }
    })
    .catch(err => console.log('Error during call', err));
