/*type actions*/
export const EDIT_FILE = 'EDIT_FILE';
export const GET_FILE = 'GET_FILE';
export const EMPTY_FILE = 'EMPTY_FILE';

/*action generators*/
export function emptyFile(list_id){
	return { type: EMPTY_FILE, list_id}
}

export function editFile(id, files) {
	return { type: EDIT_FILE, id , files}
}

export function getFile(id, files) {
	return { type: GET_FILE, id, files}
}


