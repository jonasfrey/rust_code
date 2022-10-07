import { DOMParser } from "https://deno.land/x/deno_dom/deno-dom-wasm.ts";
var s_url = "https://irsa.ipac.caltech.edu/data/ZTF/lc/lc_dr11/0/"
const o_dom_parser = new DOMParser();
var o_response = await fetch(s_url)
var s_response = await o_response.text();

const o_document = o_dom_parser.parseFromString(s_response, "text/html");

var o_nodelist_queryselectorall_a = o_document.querySelectorAll("a"); 

var a_o_url = Array.prototype.slice.call(o_document.querySelectorAll("a"))
var a_s_url = []
var n_i = 0; 
// while(n_i<a_o_url.length){
    // console.log(a_o_url[n_i])
    // a_s_url.push(a_o_url[n_i].href)

    n_i+=1;
// }
console.log(o_nodelist_queryselectorall_a[0])
// console.log(a_s_url)
// console.log(typeof o_nodelist_queryselectorall_a.map())
// var a_s_url_part = a_o_url.map(o=>o.href)
// var a_s_url_parquet = []

// for(var s_url_part of a_s_url_part){

    
//     var o_response = await fetch(s_url_part);
//     var s_response = await o_response.text()
//     const o_document = o_dom_parser.parseFromString(s_response, "text/html");

//     a_s_url_parquet = a_s_url_parquet.concat( Array.prototype.slice.call(o_document.querySelectorAll("a")).map(o=>o.href).filter(s=s.includes(".parquet")))
//     //console.log(a_s_url_parquet)
// }

// await Deno.writeTextFile(
//     "a_s_url_parquet.json", 
//     JSON.stringify(a_s_url_parquet)
// )
// console.log("done")