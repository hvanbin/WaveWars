using UnityEngine;
using System.Collections;

public class WaveSpawn : MonoBehaviour
{
    public static bool LEFT_READY = true;
    public static bool RIGHT_READY = true;
	public static int LEFT_TYPE = 0;
	public static int RIGHT_TYPE = 0;
	public static float LEFT_ROW = 0;
	public static float RIGHT_ROW = 0;
    public KeyCode key;
    public enum Type{Sawtooth, Square, Triangle};
    public Type type;
    public bool leftward;

    private GameObject spawning;
    private int spawnName;
    void Start()
    {
        switch (type)
        {
            case Type.Sawtooth:
                spawning = Resources.Load("Waves/Sawtooth") as GameObject;
                spawnName = 1;
                break;
            case Type.Square:
                spawning = Resources.Load("Waves/Square") as GameObject;
                spawnName = 2;
                break;
            default:
                spawning = Resources.Load("Waves/Triangular") as GameObject;
                spawnName = 3;
                break;
        }
	}
	void Update()
    {
        if(Input.GetKeyDown(key))
        {
            if(!leftward || Plane.AI == 0) Spawn(leftward);
        }
    }

	public void Spawn (bool leftward)
    {
		if (leftward) {
			LEFT_READY = false;
			LEFT_TYPE = spawnName;
			LEFT_ROW = transform.localPosition.z;
		}
		else { 
			RIGHT_READY = false;
			RIGHT_TYPE = spawnName;
			RIGHT_ROW = transform.localPosition.z;
		}

        /*GameObject wave = Instantiate(spawning, GameObject.FindGameObjectWithTag("Plane").transform, false) as GameObject;
        wave.name = spawnName;
        wave.transform.position = new Vector3(transform.position.x + (leftward ? -xOffset : xOffset), wave.transform.position.y, transform.position.z);
        wave.GetComponent<Wave>().setLeftward(leftward);
        wave.transform.GetChild(0).GetComponent<ParticleSystem>().startColor = leftward? Resources.Load<Material>("Materials/Gray").color : Resources.Load<Material>("Materials/Brown").color;
        wave.transform.GetChild(1).GetComponent<ParticleSystem>().startColor = leftward ? Resources.Load<Material>("Materials/Gray").color : Resources.Load<Material>("Materials/Brown").color;
*/
	}

    void OnMouseOver()
    {
        if (Input.GetMouseButtonUp(0))
        {
            if(!leftward || Plane.AI == 0) Spawn(leftward);
            /*if(leftward){if (LEFT_READY) {Spawn(); LEFT_READY = false;}}
            else{if (RIGHT_READY) {Spawn(); RIGHT_READY = false;}}*/
        }
    }
}
