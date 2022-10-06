var s_url = "https://irsa.ipac.caltech.edu/data/ZTF/lc/lc_dr11/1/"

var a_o_url = Array.prototype.slice.call(document.querySelectorAll("a"))

var a_s_url_part = a_o_url.map(o=>o.href).filter(s=>s.includes("field"))


var a_s_url_parquet = []

const parser = new DOMParser();


for(var s_url_part of a_s_url_part){

    var o_response = await fetch(s_url_part);
    var s_response = await o_response.text()
    let o_document = parser.parseFromString(s_response, "text/html");
    var o_nodelist_a = o_document.querySelectorAll("a");
    
    console.log("length")
    console.log(o_nodelist_a.length)

    a_s_url_parquet = a_s_url_parquet.concat(Array.prototype.slice.call(o_nodelist_a).map(o=>s_url_part+o.href.split("/").pop()).filter(s=>s.includes(".parquet")))
    console.log(a_s_url_parquet)
    // console.log(o_document.querySelectorAll("a")[5].href)
}
JSON.stringify(a_s_url_parquet)
