import store from './app.jsx';
import { editFile, getFile, emptyFile } from './actions/files.js';
import { addElem, deleteElem } from './actions/list.js';


function setFile(event){
	
	store.dispatch(editFile(event.target.id,event.target.files)); 
	
    var listFiles = store.getState().list_files;
    var eFiles = event.target.files;

    for(var i=0; i<eFiles.length; i++){
		
        var result = true;

        for(var j=0; j<listFiles.length; j++){
            if(listFiles[j].name == eFiles[i].name){
                result = false;
            }
        }

        if(eFiles[i].type == 'text/csv' && result == true){
		    store.dispatch(addElem(eFiles[i]));
        }

	}
	
	//console.log(store.getState().list_files);

}

function show(e){
    console.log(e);
}

export { setFile, show };
