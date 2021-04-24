using UnityEngine;
using System.Collections;

public class MenuWaves : MonoBehaviour {

	private bool leftward;
	private int C=0;

	// Use this for initialization
	void Start () {
		leftward = Random.Range (0,2)==0;
		this.transform.GetChild(0).GetComponent<ParticleSystem>().startColor = leftward? Resources.Load<Material>("Materials/Gray").color : Resources.Load<Material>("Materials/Brown").color;
		}
	
	// Update is called once per frame
	void Update () {
		if(C==0){
			leftward = Random.Range (0,2)==0;
			this.transform.GetChild(0).GetComponent<ParticleSystem>().startColor = leftward? Resources.Load<Material>("Materials/Gray").color : Resources.Load<Material>("Materials/Brown").color;
			C = Random.Range (60,180);
		}
		C--;
	}
}
