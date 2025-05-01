<script>
    import { page } from '$app/stores';
    // Example data
    let headers = ['id', 'part name', 'quantity'];
    let rows = [
      [ $page.params.id, "photo resistor sensor", 'x4'],
      [ $page.params.id, "arduino nano", 'x3'],
      [ $page.params.id, "stepper motor driver", 'x10']
    ];
  </script>
  
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
  