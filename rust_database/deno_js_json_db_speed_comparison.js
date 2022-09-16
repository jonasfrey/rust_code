import {O_json_db} from "https://deno.land/x/o_json_db@0.5/O_json_db.module.js"


class O_person{

    constructor(
        n_id,
        s_name, 
        s_email,
    ){
        this.n_id = n_id
        this.s_name = s_name
        this.s_email = s_email
    }

}


const o_json_db = new O_json_db();

var n_i = 0; 
var n_max = 100000;
while(n_i < n_max){
    var o_person = new O_person(n_i, "some body", "some@body.com");
    var o = await o_json_db.f_o_create(o_person);
    var o_person = new O_person(n_i, "other body", "other@body.com");
    var o = await o_json_db.f_o_create(o_person);

    n_i+=1;
}