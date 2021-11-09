import createActor from './actor';

const executeRequest = (data) => {
    console.log("Received request json", data);
    // TODO: add build fargate call
}

createActor().consume_request({})
    .then(json => {
        if (json['Ok']) { // list
            json['Ok'].forEach(data => executeRequest(data));
        } else {
            console.log("Error", json);
        }
    })
    .catch(err => console.log('Error during call', err));
