import json
import random
import string
import uuid
import sys


def progress_bar(current, total, width=50):
    percent = float(current) * 100 / total
    bar = "=" * int(percent / 100 * width) + "-" * (width - int(percent / 100 * width))
    print(f"\rProgress: [{bar}] {percent:.2f}%", end="")
    if current == total:
        print()


def generate_deep_nested_structure(depth=0, max_depth=100):
    if depth >= max_depth:
        return random.choice(
            [
                random.randint(1, 1000000),
                random.uniform(0, 1000000),
                "".join(random.choices(string.ascii_letters + string.digits, k=20)),
                bool(random.getrandbits(1)),
                None,
            ]
        )

    return {
        f"key_{depth}_{i}": generate_deep_nested_structure(depth + 1, max_depth)
        for i in range(random.randint(1, 5))
    }


def generate_large_array(size=100000):
    print(f"Generating large array of size {size}...")
    return [random.randint(1, 1000000) for _ in range(size)]


def generate_complex_object():
    return {
        "id": str(uuid.uuid4()),
        "name": "".join(random.choices(string.ascii_letters, k=10)),
        "value": random.uniform(0, 10000),
        "active": random.choice([True, False]),
        "tags": [
            random.choice(["red", "blue", "green", "yellow", "purple"])
            for _ in range(random.randint(1, 10))
        ],
        "nested": generate_deep_nested_structure(max_depth=10),
        "array": generate_large_array(size=1000),
    }


def generate_complex_json(num_objects=1000):
    print("Starting JSON generation...")
    data = {
        "metadata": {
            "version": "3.0",
            "description": "Extremely complex JSON for advanced parsing benchmarks",
            "generated": "2024-09-25T15:00:00Z",
        },
        "data": [],
        "statistics": {
            "object_count": num_objects,
            "total_elements": num_objects + 500000 + 200,
            "estimated_size": "10MB+",
        },
    }

    print(f"Generating {num_objects} complex objects...")
    for i in range(num_objects):
        data["data"].append(generate_complex_object())
        progress_bar(i + 1, num_objects)

    print("\nGenerating large array...")
    data["large_array"] = generate_large_array(size=500000)

    print("Generating deep structure...")
    data["deep_structure"] = generate_deep_nested_structure(max_depth=200)

    return data


# Generate the JSON
print("Initializing JSON generation process...")
complex_json = generate_complex_json(num_objects=2000)

# Convert to JSON string
print("Converting to JSON string...")
json_string = json.dumps(complex_json)

# Check the size
size_mb = len(json_string) / (1024 * 1024)
print(f"Generated JSON size: {size_mb:.2f} MB")

# Save to a file
print("Saving to file...")
with open("complex_benchmark_data.json", "w") as f:
    f.write(json_string)

print("JSON data has been generated and saved to 'complex_benchmark_data.json'")
