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
	
	console.log(store.getState().list_files);

}

function show(name){
    
    var listFiles = store.getState().list_files;
    var file = false;

    for(var i=0; i<listFiles.length; i++){
        if(listFiles[i].name == name){
            file = listFiles[i];
            break;
        }
    }

    if(file != false){
        
        var data = new FormData();
        data.append('file', file);
        data.append('time',new Date());

        $.ajax({
            url: '/upload/',
            type: 'POST',
            data: data,
            cache: false,
            dataType: 'json',
            processData: false, 
            contentType: false, 
            success: function(data){
                
                data.sort(function(a, b) { 
                    return a.class - b.class;
                });
                
                console.log(data);
                
            },
            error: function(jqXHR, textStatus, errorThrown){
             
                console.log(textStatus);
                console.log(jqXHR);
                        
            }
        });

    }

}

export { setFile, show };
