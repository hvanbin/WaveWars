using UnityEngine;
using System.Collections;

public class ColorFixer : MonoBehaviour {

	public bool side;

	// Use this for initialization
	void Update () {
		if(side){
			if(transform.GetChild(0).GetComponentInChildren<ParticleSystem>().startColor!=Resources.Load<Material> ("Materials/Gray").color){
				for (int i = 0; i < transform.childCount; i++)
				{
                    if(transform.GetChild(i).GetComponentInChildren<ParticleSystem>() != null)
                        transform.GetChild(i).GetComponentInChildren<ParticleSystem>().startColor=Resources.Load<Material> ("Materials/Gray").color;
				}
			}
		}else{
			if (transform.GetChild (0).GetComponentInChildren<ParticleSystem> ().startColor != Resources.Load<Material> ("Materials/Brown").color) {
				for (int i = 0; i < transform.childCount; i++)
                {
                    if (transform.GetChild(i).GetComponentInChildren<ParticleSystem>() != null)
                        transform.GetChild (i).GetComponentInChildren<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Brown").color;
				}
			}
		}
	}
}
