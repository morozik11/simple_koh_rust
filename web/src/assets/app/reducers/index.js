
import { combineReducers } from 'redux';
import { EDIT_FILE, GET_FILE, EMPTY_FILE } from '../actions/files.js';
import { ADD, DELETE  } from '../actions/list.js';
	
	
	
function list_files(state = [], action){
	
	switch (action.type) {
		case ADD:
            var newState = state.concat([action.file]);
            return newState;	
		default:
			return state;
	}
	
}
	
function files_inputs(state = [], action){
	
	switch (action.type) {
		case EDIT_FILE:
			
			return state.map((input) => {
				if (input.id === action.id) {
					return Object.assign({}, input, {
						id: action.id,
						files: action.files
					});
				}
				return input;
			});
			
		case GET_FILE:
			return state.map((input) => {
				if (input.id === action.id) {
					action.files = input.files;
					return input;
				}
				return input;
			});
		case EMPTY_FILE:
			return state.map((input) => {
				
				let i = action.list_id.indexOf(input.id); 
				
				if (i != -1) {
					return Object.assign({}, input, {
						id: action.list_id[i],
						files: ''
					});
				}
				
				return input;
				
			});
		default:
			return state;
			
	}
	
}


const WebApp = combineReducers({
  files_inputs,
  list_files
})


export default WebApp
