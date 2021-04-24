using UnityEngine;
using System.Collections;
using UnityEngine.UI;

public class ColorText : MonoBehaviour {

	public bool side;

	// Use this for initialization
	void Update () {
		if(side){
			if(this.GetComponent<Text>().color!=Resources.Load<Material> ("Materials/Gray").color){
					this.GetComponent<Text>().color=Resources.Load<Material> ("Materials/Gray").color;
			}
		}else{
			if (this.GetComponent<Text>().color != Resources.Load<Material> ("Materials/Brown").color) {
				this.GetComponent<Text>().color = Resources.Load<Material> ("Materials/Brown").color;
			}
		}
	}
}
