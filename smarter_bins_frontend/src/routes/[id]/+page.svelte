<script>
    import { page } from '$app/stores';
    // Example data
    let headers = ['id', 'part name', 'quantity'];
    let rows = [
      [ '1', "photo resistor sensor", 'x4'],
      [ '2', "arduino nano", 'x3'],
      [ '3', "stepper motor driver", 'x10']
    ];
  </script>
    <h1>Bin ID: {$page.params.id}</h1>
  <table>
    <thead>
      <tr>
        {#each headers as header}
          <th>{header}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each rows as row}
        <tr>
          {#each row as cell}
            <td>{cell}</td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
  <!-- <button on:click={}>Back</button> -->
   <div>
       <button>Take</button>
       <button>Refill</button>
   </div>
  <style>
    table {
      border-collapse: collapse;
      width: 100%;
    }
    th, td {
      border: 1px solid #ccc;
      padding: 0.5rem;
      text-align: left;
    }
    th {
      background-color: #f4f4f4;
    }
  </style>
  