class O_object_property{
    constructor(
        o,
    ){
        var a_s_prop_name_allowed = [
            "s_name",        // "s_name", "n_id", ...
            "o_type",        // O_object_property_type
        ]
        for(var s_prop_name in o){
            if(a_s_prop_name_allowed.includes(s_prop_name)){
                this[s_prop_name] = o[s_prop_name]
            }else{
                console.log(`s_prop_name ${s_prop_name} is not allowed`)
            }
        }
    }

}
export {O_object_property}